
int __thiscall
FUN_0054c590(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5,undefined4 param_6,void *param_7,void *param_8)

{
  void *pvVar1;
  void *pvVar2;
  bool bVar3;
  uint *puVar4;
  undefined3 extraout_var;
  int iVar5;
  undefined4 local_60;
  undefined4 local_5c;
  code *local_50;
  code *pcStack_4c;
  undefined4 auStack_48 [15];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar2 = param_8;
  puStack_8 = &LAB_006480b0;
  local_c = ExceptionList;
  local_60 = 1;
  local_5c = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_8,&local_60);
  local_4 = 0xffffffff;
  FUN_00619730();
  pvVar1 = param_7;
  puVar4 = (uint *)FUN_00402e40(param_7,(uint *)&param_7);
  local_4 = 1;
  bVar3 = FUN_0054c720(this,puVar4,&local_50);
  local_4 = 0xffffffff;
  FUN_00619730();
  param_8 = (void *)0x0;
  iVar5 = 0;
  if (CONCAT31(extraout_var,bVar3) != 0) {
    iVar5 = (*local_50)(param_1,param_6,&param_8,pvVar2);
    if ((iVar5 != 0) && (*(int *)((int)pvVar2 + 4) == -1)) {
      FUN_004f3f70(&local_60);
      local_4 = 2;
      FUN_00582b90(auStack_48,1,1,1,param_1,pvVar1,&local_60,&local_60,&local_60,param_2,param_3,
                   param_4,param_5,param_8,param_6);
      local_4._0_1_ = 3;
      iVar5 = (*pcStack_4c)(auStack_48,pvVar2,&param_7);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00582c20(auStack_48);
      local_4 = 0xffffffff;
      FUN_004f3fb0(&local_60);
    }
  }
  ExceptionList = local_c;
  return iVar5;
}

