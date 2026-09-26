
void __fastcall FUN_005d6b40(int param_1)

{
  int iVar1;
  
  DAT_006bcbb0 = FUN_005d6850(*(undefined4 *)(param_1 + 0xfc));
  DAT_006bcbb4 = (**(code **)(**(int **)(param_1 + 0xf0) + 0xc))
                           (*(int **)(param_1 + 0xf0),FUN_005d6aa0,param_1 + 0x100);
  if (DAT_006bcbb4 != 0) {
    FUN_005d93f0(0,DAT_006bcbb4);
  }
  iVar1 = FUN_005f5060(param_1 + 0x100);
  *(int *)(param_1 + 0x108) = iVar1;
  if (iVar1 == 0) {
    FUN_005d93f0(param_1,0);
  }
  return;
}

