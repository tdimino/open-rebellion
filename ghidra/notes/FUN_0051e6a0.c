
uint __thiscall FUN_0051e6a0(void *this,int *param_1)

{
  int *piVar1;
  bool bVar2;
  int iVar3;
  uint uVar4;
  undefined3 extraout_var;
  int iStack_20;
  undefined4 uStack_1c;
  int iStack_18;
  int iStack_14;
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  uint uVar5;
  
  piVar1 = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006432b8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar3 = (**(code **)(*param_1 + 0x10))();
  uVar5 = 0;
  if (iVar3 == 2) {
    FUN_00568fe0(&iStack_20);
    uStack_4 = 0;
    iStack_20 = (**(code **)(*piVar1 + 0x10))();
    uStack_1c = (**(code **)(*piVar1 + 0xc))();
    iStack_18 = piVar1[6];
    iStack_14 = piVar1[4];
    iStack_10 = piVar1[5];
    uVar4 = FUN_0051e910(this,&iStack_20,&param_1);
    uVar5 = 0;
    if (uVar4 != 0) {
      piVar1[3] = (int)param_1;
      bVar2 = FUN_0051eaa0(this,piVar1);
      uVar5 = CONCAT31(extraout_var,bVar2);
    }
    uStack_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return uVar5;
}

