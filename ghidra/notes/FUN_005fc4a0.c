
void __fastcall FUN_005fc4a0(int *param_1)

{
  ushort uVar1;
  int iVar2;
  
  uVar1 = *(ushort *)(*param_1 + 0xe);
  if (uVar1 < 0x18) {
    iVar2 = *(int *)(*param_1 + 0x20);
    param_1[4] = iVar2;
    if (iVar2 == 0) {
      param_1[4] = 1 << ((byte)uVar1 & 0x1f);
      return;
    }
  }
  else {
    param_1[4] = 0;
  }
  return;
}

