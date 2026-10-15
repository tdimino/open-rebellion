
void __thiscall FUN_004f8680(int *param_1,void *param_2)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  void *unaff_retaddr;
  int *piStack_4;
  
  piStack_4 = param_1;
  FUN_004f6b50(param_1,(int *)&piStack_4);
  uVar3 = param_1[0x14];
  if (((uVar3 & 4) == 0) || ((uVar3 & 0x20) != 0)) {
    if ((uVar3 & 8) == 0) {
      FUN_004f78e0(param_1,1,param_2);
    }
  }
  else {
    uVar3 = (uint)piStack_4[9] >> 6 & 3;
    if ((uVar3 == 0) || (3 < uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    if (bVar1) {
      iVar2 = (**(code **)(*param_1 + 0x50))(uVar3);
      if (iVar2 != 0) {
        FUN_004f6f40(param_1,uVar3,unaff_retaddr);
        return;
      }
      (**(code **)(*param_1 + 0xac))(3,unaff_retaddr);
      return;
    }
  }
  return;
}

