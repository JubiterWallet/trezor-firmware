from typing import TYPE_CHECKING

import storage.recovery
from trezor import ui, wire
from trezor.enums import ButtonRequestType
from trezor.ui.layouts import confirm_action, show_success, show_warning
from trezor.ui.layouts.common import button_request
from trezor.ui.layouts.recovery import (  # noqa: F401
    continue_recovery,
    request_word,
    request_word_count,
    show_group_share_success,
    show_remaining_shares,
)

from .. import backup_types
from . import word_validity
from .recover import RecoveryAborted

if TYPE_CHECKING:
    from typing import Callable
    from trezor.enums import BackupType


async def confirm_abort(dry_run: bool = False) -> None:
    if dry_run:
        await confirm_action(
            "abort_recovery",
            "Abort seed check",
            "Do you really want to abort the seed check?",
            br_code=ButtonRequestType.ProtectCall,
        )
    else:
        await confirm_action(
            "abort_recovery",
            "Abort recovery",
            action="Do you really want to abort the recovery process?",
            description="All progress will be lost.",
            br_code=ButtonRequestType.ProtectCall,
        )


async def request_mnemonic(
    word_count: int, backup_type: BackupType | None
) -> str | None:
    await button_request(
        wire.get_context(), "mnemonic", code=ButtonRequestType.MnemonicInput
    )

    words: list[str] = []
    for i in range(word_count):
        word = await request_word(
            i, word_count, is_slip39=backup_types.is_slip39_word_count(word_count)
        )
        words.append(word)

        try:
            word_validity.check(backup_type, words)
        except word_validity.AlreadyAdded:
            await show_share_already_added()
            return None
        except word_validity.IdentifierMismatch:
            await show_identifier_mismatch()
            return None
        except word_validity.ThresholdReached:
            await show_group_threshold_reached()
            return None

    return " ".join(words)


async def show_dry_run_result(result: bool, is_slip39: bool) -> None:
    if result:
        if is_slip39:
            text = "The entered recovery\nshares are valid and\nmatch what is currently\nin the device."
        else:
            text = "The entered recovery\nseed is valid and\nmatches the one\nin the device."
        await show_success("success_dry_recovery", text, button="Continue")
    else:
        if is_slip39:
            text = "The entered recovery\nshares are valid but\ndo not match what is\ncurrently in the device."
        else:
            text = "The entered recovery\nseed is valid but does\nnot match the one\nin the device."
        await show_warning("warning_dry_recovery", text, button="Continue")


async def show_dry_run_different_type() -> None:
    await show_warning(
        "warning_dry_recovery",
        header="Dry run failure",
        content="Seed in the device was\ncreated using another\nbackup mechanism.",
        icon=ui.ICON_CANCEL,
        icon_color=ui.ORANGE_ICON,
        br_code=ButtonRequestType.ProtectCall,
    )


async def show_invalid_mnemonic(word_count: int) -> None:
    if backup_types.is_slip39_word_count(word_count):
        await show_warning(
            "warning_invalid_share",
            "You have entered\nan invalid recovery\nshare.",
        )
    else:
        await show_warning(
            "warning_invalid_seed",
            "You have entered\nan invalid recovery\nseed.",
        )


async def show_share_already_added() -> None:
    await show_warning(
        "warning_known_share",
        "Share already entered,\nplease enter\na different share.",
    )


async def show_identifier_mismatch() -> None:
    await show_warning(
        "warning_mismatched_share",
        "You have entered\na share from another\nShamir Backup.",
    )


async def show_group_threshold_reached() -> None:
    await show_warning(
        "warning_group_threshold",
        "Threshold of this\ngroup has been reached.\nInput share from\ndifferent group.",
    )


async def homescreen_dialog(
    button_label: str,
    text: str,
    subtext: str | None = None,
    info_func: Callable | None = None,
) -> None:
    while True:
        if await continue_recovery(button_label, text, subtext, info_func):
            # go forward in the recovery process
            break
        # user has chosen to abort, confirm the choice
        dry_run = storage.recovery.is_dry_run()
        try:
            await confirm_abort(dry_run)
        except wire.ActionCancelled:
            pass
        else:
            raise RecoveryAborted
