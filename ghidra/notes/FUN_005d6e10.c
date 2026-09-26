
void __fastcall FUN_005d6e10(int param_1)

{
  int iVar1;
  
  iVar1 = -0x7fffbffb;
  if (*(int *)(param_1 + 0x120) != 0) {
    iVar1 = (**(code **)(**(int **)(param_1 + 0xf0) + 0x20))
                      (*(int **)(param_1 + 0xf0),&DAT_0066e468,*(undefined4 *)(param_1 + 0x158),
                       param_1 + 0x134);
  }
  if (iVar1 != 0) {
    DAT_006bcbb4 = (**(code **)(**(int **)(param_1 + 0xf0) + 0x20))
                             (*(int **)(param_1 + 0xf0),&DAT_0066e458,
                              *(undefined4 *)(param_1 + 0x158),param_1 + 0x134);
    if (DAT_006bcbb4 != 0) {
      FUN_005d93f0(0,DAT_006bcbb4);
    }
  }
  DAT_006bcbb4 = (**(code **)(**(int **)(param_1 + 0x128) + 0x40))
                           (*(int **)(param_1 + 0x128),*(undefined4 *)(param_1 + 0xf0),
                            *(undefined4 *)(param_1 + 0x134),param_1 + 0x130);
  if (DAT_006bcbb4 != 0) {
    FUN_005d93f0(0,DAT_006bcbb4);
  }
  return;
}

