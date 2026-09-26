
int __thiscall FUN_0058fc90(void *this,int *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int *this_00;
  undefined4 *puVar2;
  int local_74;
  int local_70 [5];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650458;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  local_4 = 0;
  local_74 = 0;
  bVar1 = FUN_0058fa30(this,param_1,&local_74);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    puVar2 = (undefined4 *)((int)this + 0x24);
    this_00 = FUN_00584e60(param_1,local_70,&local_74);
    FUN_00617610(this_00,puVar2);
  }
  FUN_00584fd0(param_1,&local_74);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return CONCAT31(extraout_var,bVar1);
}

