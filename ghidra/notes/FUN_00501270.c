
int __thiscall FUN_00501270(int *param_1,undefined4 param_2,undefined4 param_3)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  undefined4 unaff_EBP;
  undefined4 unaff_retaddr;
  
  iVar2 = (**(code **)(*param_1 + 0x1fc))(param_2,param_3);
  iVar3 = FUN_00500670();
  if (iVar3 != 0) {
    iVar3 = (**(code **)(*param_1 + 0x248))();
    if (iVar3 != 0) {
      uVar1 = param_1[0x19];
      iVar3 = FUN_00500670();
      iVar4 = FUN_00500670();
      iVar2 = FUN_0053e170(iVar2,iVar3 - (uVar1 >> 4 & 0xf),iVar4);
      iVar3 = (**(code **)(*param_1 + 0x274))();
      iVar4 = (**(code **)(*param_1 + 0x248))();
      iVar5 = (**(code **)(*param_1 + 0x248))();
      iVar2 = FUN_0053e170(iVar2,iVar4 - iVar3,iVar5);
    }
  }
  iVar3 = (**(code **)(*param_1 + 0x1fc))(unaff_EBP,unaff_retaddr);
  return iVar3 - iVar2;
}

