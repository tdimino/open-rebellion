
undefined4 __thiscall FUN_00590830(void *this,int *param_1,int *param_2)

{
  int *piVar1;
  undefined2 *puVar2;
  undefined4 *puVar3;
  int local_70 [5];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650578;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  local_4 = 0;
  FUN_00584d80(this,param_1,param_2);
  puVar3 = (undefined4 *)((int)this + 0x40);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar1,puVar3);
  puVar3 = (undefined4 *)((int)this + 0x44);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar1,puVar3);
  puVar2 = (undefined2 *)((int)this + 0x48);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617900(piVar1,puVar2);
  puVar2 = (undefined2 *)((int)this + 0x4a);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617900(piVar1,puVar2);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return 1;
}

