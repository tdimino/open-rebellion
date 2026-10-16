
void __fastcall FUN_004e69d0(int param_1)

{
  int iVar1;
  ushort uVar2;
  
  uVar2 = FUN_005f50e0(*(int *)(param_1 + 0xc) + 0x11c);
  iVar1 = *(int *)(param_1 + 0xc);
  if ((int)(uint)uVar2 < *(int *)(iVar1 + 0x224)) {
    FUN_0042ed50((void *)(iVar1 + 0x11c));
    return;
  }
  if (*(int *)(iVar1 + 0x224) < (int)(uint)uVar2) {
    FUN_0042edc0((void *)(iVar1 + 0x11c));
  }
  return;
}

