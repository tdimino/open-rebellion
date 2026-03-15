
void __fastcall FUN_005d9250(int *param_1)

{
  int iVar1;
  
  if (param_1[0xe] != 1) {
    (**(code **)(*param_1 + 0x30))();
    iVar1 = DAT_006bcbe0;
    if ((param_1[0xf] != 1) && (iVar1 = DAT_006bcbe4, param_1[0xf] != 2)) {
      iVar1 = DAT_006bcbe8;
    }
    iVar1 = (**(code **)(*(int *)param_1[0xb] + 0x48))((int *)param_1[0xb],iVar1,0);
    FUN_005d8b00(iVar1);
    iVar1 = DAT_006bcbe0;
    if (param_1[0xf] == 1) {
      param_1[0xe] = 1;
      param_1[0xd] = iVar1;
      return;
    }
    iVar1 = DAT_006bcbe4;
    if (param_1[0xf] != 2) {
      iVar1 = DAT_006bcbe8;
    }
    param_1[0xd] = iVar1;
    param_1[0xe] = 1;
  }
  return;
}

