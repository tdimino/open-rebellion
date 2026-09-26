
undefined4 __fastcall FUN_005f70e0(int param_1)

{
  int iVar1;
  undefined2 extraout_var;
  int iVar2;
  undefined2 extraout_var_00;
  int iVar3;
  undefined4 uVar4;
  
  uVar4 = 0;
  _SmackSoundUseDirectSound_4(0);
  iVar1 = _SmackOpen_12(param_1 + 0x94,*(undefined4 *)(param_1 + 0x1b8),0xffffffff);
  *(int *)(param_1 + 0x1ac) = iVar1;
  if (iVar1 != 0) {
    iVar2 = CONCAT22(extraout_var,*(undefined2 *)(iVar1 + 8));
    iVar3 = CONCAT22(extraout_var_00,*(undefined2 *)(param_1 + 0x19c));
    iVar1 = CONCAT22((short)((uint)iVar1 >> 0x10),*(undefined2 *)(iVar1 + 4));
    iVar1 = _SmackBufferOpen_24(*(undefined4 *)(param_1 + 0x18),3,iVar1,iVar2,iVar1 * iVar3,
                                iVar2 * iVar3);
    *(int *)(param_1 + 0x1a8) = iVar1;
    SetWindowPos(*(HWND *)(param_1 + 0x18),(HWND)0x0,0,0,
                 (uint)*(ushort *)(iVar1 + 0x10) * *(int *)(param_1 + 0x19c),
                 (uint)*(ushort *)(iVar1 + 0x14) * *(int *)(param_1 + 0x19c),0x42);
    if (*(int *)(param_1 + 0x1a8) != 0) {
      _SmackBufferSetPalette_4(*(int *)(param_1 + 0x1a8));
    }
    uVar4 = 1;
  }
  return uVar4;
}

