
uint __fastcall FUN_00503d90(int *param_1)

{
  bool bVar1;
  uint uVar2;
  undefined3 extraout_var;
  int iVar3;
  void *unaff_retaddr;
  int iStack_4;
  
  uVar2 = 1;
  if (((*(byte *)(param_1 + 0x1e) & 0x80) == 0) && ((*(byte *)(param_1 + 0x14) & 8) == 0)) {
    iStack_4 = 0;
    bVar1 = FUN_004f6b50(param_1,&iStack_4);
    uVar2 = (uint)(CONCAT31(extraout_var,bVar1) != 0);
    if (iStack_4 != 0) {
      uVar2 = *(uint *)(iStack_4 + 0x24) >> 6 & 3;
      if ((uVar2 == 0) || (3 < uVar2)) {
        return 0;
      }
      iVar3 = (**(code **)(*param_1 + 0x50))(uVar2);
      if (iVar3 != 0) {
        uVar2 = FUN_004f6f40(param_1,uVar2,unaff_retaddr);
        return uVar2;
      }
      uVar2 = (**(code **)(*param_1 + 0xac))(3,unaff_retaddr);
    }
  }
  return uVar2;
}

