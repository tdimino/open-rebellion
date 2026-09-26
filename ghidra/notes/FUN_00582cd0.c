
int __cdecl FUN_00582cd0(uint *param_1,undefined4 *param_2,undefined4 *param_3,undefined4 *param_4)

{
  bool bVar1;
  bool bVar2;
  undefined3 extraout_var;
  uint uVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  undefined4 *puVar3;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064eff0;
  local_c = ExceptionList;
  bVar1 = false;
  ExceptionList = &local_c;
  *param_2 = 0;
  *param_3 = 0;
  *param_4 = 0;
  bVar2 = FUN_00582c30(param_1,param_2);
  puVar3 = (undefined4 *)CONCAT31(extraout_var,bVar2);
  if (puVar3 != (undefined4 *)0x0) {
    local_4 = 0;
    uVar4 = (**(code **)(*(int *)*param_2 + 4))();
    if ((0x3b < uVar4) && (uVar4 < 0x40)) {
      bVar1 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      *param_3 = *param_2;
    }
    else {
      bVar1 = true;
      local_4 = 1;
      uVar4 = (**(code **)(*(int *)*param_2 + 4))();
      if ((uVar4 < 0x30) || (0x3b < uVar4)) {
        bVar1 = false;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      if (bVar1) {
        *param_4 = *param_2;
      }
      else {
        param_2 = (undefined4 *)0x0;
        puVar3 = param_2;
      }
    }
  }
  param_2 = puVar3;
  ExceptionList = local_c;
  return (int)param_2;
}

