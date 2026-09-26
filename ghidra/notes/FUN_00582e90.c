
int __cdecl FUN_00582e90(uint *param_1,undefined4 *param_2,undefined4 *param_3,undefined4 *param_4)

{
  bool bVar1;
  undefined4 *puVar2;
  bool bVar3;
  undefined3 extraout_var;
  uint *puVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  undefined4 *puVar4;
  
  puVar2 = param_3;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f030;
  local_c = ExceptionList;
  bVar1 = false;
  ExceptionList = &local_c;
  *param_2 = 0;
  *param_3 = 0;
  *param_4 = 0;
  bVar3 = FUN_00582dd0(param_1,param_2);
  puVar4 = (undefined4 *)CONCAT31(extraout_var,bVar3);
  if (puVar4 != (undefined4 *)0x0) {
    puVar5 = (uint *)FUN_00402e40((void *)*param_2,(uint *)&param_3);
    local_4 = 0;
    if ((0x3b < *puVar5 >> 0x18) && (*puVar5 >> 0x18 < 0x40)) {
      bVar1 = true;
    }
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      *puVar2 = *param_2;
    }
    else {
      puVar5 = (uint *)FUN_00402e40((void *)*param_2,(uint *)&param_3);
      bVar1 = true;
      local_4 = 1;
      if ((*puVar5 >> 0x18 < 0x30) || (0x3b < *puVar5 >> 0x18)) {
        bVar1 = false;
      }
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00619730();
      if (bVar1) {
        *param_4 = *param_2;
      }
      else {
        param_2 = (undefined4 *)0x0;
        puVar4 = param_2;
      }
    }
  }
  param_2 = puVar4;
  ExceptionList = local_c;
  return (int)param_2;
}

