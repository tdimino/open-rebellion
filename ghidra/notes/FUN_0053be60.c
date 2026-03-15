
undefined4 __thiscall FUN_0053be60(void *this,int *param_1,int *param_2)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  int *piVar4;
  int iVar5;
  undefined4 *puVar6;
  int iVar7;
  undefined4 *puVar8;
  int local_70 [5];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar3 = param_2;
  piVar4 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00646358;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  local_4 = 0;
  FUN_005852f0(this,param_1,param_2);
  puVar6 = (undefined4 *)((int)this + 0x60);
  piVar2 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar2,puVar6);
  puVar6 = (undefined4 *)((int)this + 100);
  piVar2 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar2,puVar6);
  puVar6 = (undefined4 *)((int)this + 0x68);
  piVar2 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar2,puVar6);
  puVar6 = (undefined4 *)((int)this + 0x6c);
  piVar2 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar2,puVar6);
  puVar6 = (undefined4 *)((int)this + 0x70);
  piVar2 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar2,puVar6);
  puVar6 = (undefined4 *)((int)this + 0x74);
  param_1 = (int *)0x4;
  do {
    param_2 = (int *)0x3;
    do {
      puVar8 = puVar6;
      piVar2 = FUN_00584e60(piVar4,local_70,piVar3);
      FUN_00617610(piVar2,puVar8);
      puVar6 = puVar6 + 1;
      param_2 = (int *)((int)param_2 + -1);
    } while (param_2 != (int *)0x0);
    param_1 = (int *)((int)param_1 + -1);
  } while (param_1 != (int *)0x0);
  puVar6 = (undefined4 *)((int)this + 0xa4);
  param_2 = (int *)0x3;
  do {
    puVar8 = puVar6;
    piVar2 = FUN_00584e60(piVar4,local_70,piVar3);
    FUN_00617610(piVar2,puVar8);
    puVar6 = puVar6 + 1;
    param_2 = (int *)((int)param_2 + -1);
  } while (param_2 != (int *)0x0);
  piVar4 = (int *)((int)this + 0x74);
  iVar7 = 4;
  do {
    piVar3 = (int *)((int)this + 0xb0);
    iVar5 = 3;
    do {
      iVar1 = *piVar4;
      piVar4 = piVar4 + 1;
      *piVar3 = *piVar3 + iVar1;
      piVar3 = piVar3 + 1;
      iVar5 = iVar5 + -1;
    } while (iVar5 != 0);
    iVar7 = iVar7 + -1;
  } while (iVar7 != 0);
  piVar4 = (int *)((int)this + 0x74);
  iVar7 = 4;
  do {
    iVar5 = 3;
    do {
      iVar1 = *piVar4;
      piVar4 = piVar4 + 1;
      iVar5 = iVar5 + -1;
      *(int *)((int)this + 0xbc) = *(int *)((int)this + 0xbc) + iVar1;
    } while (iVar5 != 0);
    iVar7 = iVar7 + -1;
  } while (iVar7 != 0);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return 1;
}

