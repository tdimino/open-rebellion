
bool __thiscall FUN_004f9190(void *this,int *param_1,void *param_2)

{
  void *pvVar1;
  int *piVar2;
  int *piVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  int iVar4;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  undefined3 extraout_var_04;
  undefined3 extraout_var_05;
  void *pvVar5;
  bool bVar6;
  bool bVar7;
  undefined4 auStack_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0063fed8;
  pvStack_c = ExceptionList;
  bVar7 = true;
  if ((((int)param_1 < 1) || (3 < (int)param_1)) || ((*(uint *)((int)this + 0x24) & 0x30) != 0)) {
    bVar7 = false;
  }
  bVar6 = false;
  piVar2 = param_1;
  ExceptionList = &pvStack_c;
  if (bVar7) {
    ExceptionList = &pvStack_c;
    piVar2 = (int *)(**(code **)(*(int *)this + 0xe4))();
    bVar6 = piVar2 != (int *)0x0;
  }
  bVar7 = false;
  piVar3 = param_1;
  if (bVar6) {
    piVar3 = (int *)(**(code **)(*(int *)this + 0xe4))();
    bVar7 = piVar3 != (int *)0x0;
  }
  bVar6 = false;
  if (bVar7) {
    bVar7 = FUN_0053f660(this);
    bVar6 = FUN_0053f760(piVar2,(int)this,1);
    if ((CONCAT31(extraout_var_00,bVar6) == 0) || (CONCAT31(extraout_var,bVar7) == 0)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
    bVar6 = FUN_0053f760(piVar3,(int)this,2);
    if ((CONCAT31(extraout_var_01,bVar6) == 0) || (!bVar7)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
    iVar4 = FUN_004fc5e0((int)this);
    if ((iVar4 == 0) || (!bVar7)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
    bVar6 = FUN_004fc5c0(this);
    if ((CONCAT31(extraout_var_02,bVar6) == 0) || (!bVar7)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
    bVar6 = FUN_0053f370(this);
    if ((CONCAT31(extraout_var_03,bVar6) == 0) || (!bVar7)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
    bVar6 = FUN_0053f370(piVar2);
    if ((CONCAT31(extraout_var_04,bVar6) == 0) || (!bVar7)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
    bVar6 = FUN_0053f370(piVar3);
    pvVar1 = param_2;
    if ((CONCAT31(extraout_var_05,bVar6) == 0) || (!bVar7)) {
      bVar6 = false;
    }
    else {
      bVar6 = true;
    }
    FUN_004f6f40(this,(uint)param_1,param_2);
    param_1 = *(int **)(*(int *)((int)this + 0x2c) + 0x30);
    pvVar5 = FUN_005f2fc0(auStack_18,&param_1);
    uStack_4 = 0;
    FUN_004f6e60(this,(int)pvVar5,pvVar1);
    uStack_4 = 0xffffffff;
    FUN_005f2ff0(auStack_18);
  }
  ExceptionList = pvStack_c;
  return bVar6;
}

