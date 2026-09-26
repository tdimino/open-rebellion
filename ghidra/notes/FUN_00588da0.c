
bool __thiscall FUN_00588da0(void *this,int *param_1,int *param_2)

{
  uint uVar1;
  int *this_00;
  int *piVar2;
  int iVar3;
  uint *puVar4;
  int *piVar5;
  int **ppiVar6;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  piVar2 = param_2;
  this_00 = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0064fa18;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  ppiVar6 = &param_2;
  uVar1 = param_1[9];
  param_2 = (int *)0x0;
  piVar5 = piVar2;
  iVar3 = (**(code **)(*param_1 + 0x1f8))();
  iVar3 = FUN_0055e520(uVar1 >> 6 & 3,iVar3,piVar5,(int *)ppiVar6);
  FUN_005897b0();
  if (*piVar2 != 0) {
    puVar4 = FUN_004025b0(this_00,(uint *)&param_1);
    uStack_4 = 0;
    FUN_004f26d0((void *)((int)this + 0x38),puVar4);
    uStack_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return iVar3 != 0;
}

