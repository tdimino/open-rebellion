
undefined4 __thiscall FUN_00584d80(void *this,int *param_1,int *param_2)

{
  int *piVar1;
  int *piVar2;
  undefined2 *puVar3;
  int local_74;
  int local_70 [5];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f3d8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  piVar2 = &local_74;
  local_4 = 0;
  *(undefined4 *)((int)this + 0x20) = 1;
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617760(piVar1,piVar2);
  FUN_005f5600(this,local_74);
  piVar2 = (int *)((int)this + 0x2c);
  piVar1 = FUN_00584e60(param_1,local_70,param_2);
  FUN_00617760(piVar1,piVar2);
  puVar3 = (undefined2 *)((int)this + 0x30);
  piVar2 = FUN_00584e60(param_1,local_70,param_2);
  FUN_006176a0(piVar2,puVar3);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return 1;
}

