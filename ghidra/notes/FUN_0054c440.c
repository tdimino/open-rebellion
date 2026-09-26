
int __thiscall
FUN_0054c440(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5,void *param_6,void *param_7)

{
  void *pvVar1;
  bool bVar2;
  uint *puVar3;
  undefined3 extraout_var;
  int iVar4;
  undefined4 local_60;
  undefined4 local_5c;
  undefined4 local_50;
  code *local_4c;
  undefined4 local_48 [15];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_7;
  puStack_8 = &LAB_00648080;
  local_c = ExceptionList;
  local_60 = 1;
  local_5c = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_7,&local_60);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar3 = (uint *)FUN_00402e40(param_6,(uint *)&param_7);
  local_4 = 1;
  bVar2 = FUN_0054c720(this,puVar3,&local_50);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar4 = 0;
  if (CONCAT31(extraout_var,bVar2) != 0) {
    FUN_004f3f70(&local_60);
    local_4 = 2;
    FUN_00582b90(local_48,1,1,0,param_1,param_6,&local_60,&local_60,&local_60,param_2,param_3,
                 param_4,param_5,0,0);
    local_4._0_1_ = 3;
    iVar4 = (*local_4c)(local_48,pvVar1,&param_7);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00582c20(local_48);
    local_4 = 0xffffffff;
    FUN_004f3fb0(&local_60);
  }
  ExceptionList = local_c;
  return iVar4;
}

