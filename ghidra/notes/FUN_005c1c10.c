
void __fastcall FUN_005c1c10(int *param_1)

{
  int iVar1;
  
  FUN_005d6ef0(param_1);
  DAT_006bcbd0 = param_1[0x4a];
  DAT_006bcbd4 = param_1;
  iVar1 = FUN_00595c40();
  if (iVar1 == 0) {
    FUN_00601c90((void *)param_1[0x18b],0x20000ff,(HWND)0x0);
    iVar1 = 0x200ff00;
  }
  else {
    FUN_00601c90((void *)param_1[0x18b],0x200ff00,(HWND)0x0);
    iVar1 = 0x20000ff;
  }
  FUN_00601c90((void *)param_1[0x18c],iVar1,(HWND)0x0);
  DAT_006bcbb4 = (**(code **)(*(int *)param_1[0x4c] + 0x4c))((int *)param_1[0x4c],0);
  if (DAT_006bcbb4 != 0) {
    FUN_005d93f0(0,DAT_006bcbb4);
  }
  if (param_1[0x1a1] != 0) {
    FUN_005d39a0();
  }
  FUN_005d8c30();
  FUN_005fbbb0((int)(param_1 + 0x198));
  param_1[0x196] = 1;
  return;
}

