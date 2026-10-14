
BOOL __fastcall FUN_0060fa80(int param_1)

{
  int iVar1;
  int iVar2;
  BOOL BVar3;
  int iVar4;
  int nHeight;
  int iStack_8;
  
  iVar1 = *(int *)(param_1 + 0x20);
  if (iVar1 != 0) {
    if ((*(int *)(param_1 + 0xd4) != 0) && (*(int *)(param_1 + 0xd8) != 0)) {
      iVar2 = *(int *)(*(int *)(param_1 + 0xd8) + 0x34);
      iVar4 = *(int *)(iVar1 + 0x3c) - iVar2;
      nHeight = *(int *)(iVar1 + 0x3c);
      iStack_8 = iVar4;
      if (*(int *)(*(int *)(param_1 + 0x98) + 8) != 0) {
        iStack_8 = iVar4 - iVar2;
        nHeight = iVar4;
      }
      BVar3 = MoveWindow(*(HWND *)(param_1 + 0x18),
                         *(int *)(iVar1 + 0x38) - (uint)*(ushort *)(param_1 + 0xac),0,
                         (uint)*(ushort *)(param_1 + 0xac),nHeight,1);
      if (BVar3 != 0) {
        *(int *)(param_1 + 0x34) = nHeight;
        iVar1 = *(int *)(param_1 + 0xd4);
        *(uint *)(param_1 + 0x30) = (uint)*(ushort *)(param_1 + 0xac);
        BVar3 = MoveWindow(*(HWND *)(iVar1 + 0x18),
                           (uint)(*(ushort *)(param_1 + 0xac) >> 1) - *(int *)(iVar1 + 0x30) / 2,0,
                           *(int *)(iVar1 + 0x30),*(int *)(iVar1 + 0x34),1);
        if (BVar3 != 0) {
          iVar1 = *(int *)(param_1 + 0xd8);
          BVar3 = MoveWindow(*(HWND *)(iVar1 + 0x18),
                             (uint)(*(ushort *)(param_1 + 0xac) >> 1) - *(int *)(iVar1 + 0x30) / 2,
                             iStack_8,*(int *)(iVar1 + 0x30),*(int *)(iVar1 + 0x34),1);
          return BVar3;
        }
      }
    }
  }
  return 0;
}

