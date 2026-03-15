
void __fastcall FUN_005d6a40(int param_1)

{
  int iVar1;
  
  DAT_006bcbb4 = (**(code **)(**(int **)(param_1 + 0x14c) + 0x20))
                           (*(int **)(param_1 + 0x14c),0,0,(void *)(param_1 + 0x10c),&LAB_005d69c0);
  if (DAT_006bcbb4 != 0) {
    FUN_005d93f0(0,DAT_006bcbb4);
  }
  iVar1 = FUN_005eedc0((void *)(param_1 + 0x10c),*(int *)(param_1 + 0xf4),*(int *)(param_1 + 0xf8),
                       *(int *)(param_1 + 0xfc),1);
  *(int *)(param_1 + 0x114) = iVar1;
  return;
}

