
int __thiscall FUN_00509460(void *this,undefined4 param_1,int *param_2)

{
  int *piVar1;
  int *piVar2;
  void *this_00;
  int iVar3;
  int iVar4;
  int iVar5;
  int **ppiVar6;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar2 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641978;
  pvStack_c = ExceptionList;
  iVar5 = 1;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  FUN_004ffef0(local_2c,this,1,param_1);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  do {
    if (local_10 == 0) break;
    ppiVar6 = &param_2;
    param_2 = (int *)0x0;
    iVar5 = 2;
    this_00 = (void *)FUN_0052bed0((int)local_2c);
    iVar5 = FUN_004fd790(this_00,iVar5,(int *)ppiVar6);
    if ((iVar5 != 0) && (param_2 != (int *)0x0)) {
      piVar1 = (int *)*piVar2;
      if (piVar1 != (int *)0x0) {
        iVar3 = (**(code **)(*piVar1 + 500))();
        iVar4 = (**(code **)(*param_2 + 500))();
        if (iVar4 <= iVar3) goto LAB_005094fb;
      }
      *piVar2 = (int)param_2;
    }
LAB_005094fb:
    FUN_005130d0((int)local_2c);
  } while (iVar5 != 0);
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  ExceptionList = pvStack_c;
  return iVar5;
}

