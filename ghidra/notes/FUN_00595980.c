
undefined4 __thiscall FUN_00595980(void *this,int *param_1,int *param_2)

{
  int *piVar1;
  undefined4 *puVar2;
  int local_70 [5];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650cd8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  local_4 = 0;
  FUN_00593f90(this,param_1,param_2);
  puVar2 = (undefined4 *)((int)this + 0x98);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar1,puVar2);
  puVar2 = (undefined4 *)((int)this + 0x9c);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar1,puVar2);
  puVar2 = (undefined4 *)((int)this + 0xa0);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar1,puVar2);
  puVar2 = (undefined4 *)((int)this + 0xa4);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar1,puVar2);
  puVar2 = (undefined4 *)((int)this + 0xa8);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar1,puVar2);
  puVar2 = (undefined4 *)((int)this + 0xac);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar1,puVar2);
  puVar2 = (undefined4 *)((int)this + 0xb0);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar1,puVar2);
  puVar2 = (undefined4 *)((int)this + 0xb4);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar1,puVar2);
  puVar2 = (undefined4 *)((int)this + 0xb8);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617610(piVar1,puVar2);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return 1;
}

