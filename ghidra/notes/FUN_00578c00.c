
uint __thiscall FUN_00578c00(void *this,void *param_1)

{
  void *pvVar1;
  bool bVar2;
  uint *puVar3;
  undefined3 extraout_var;
  uint uVar4;
  int *local_18;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  puStack_8 = &LAB_0064d128;
  local_c = ExceptionList;
  uVar4 = 1;
  local_18 = (int *)0x0;
  local_14[0] = 1;
  local_14[1] = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (*(int *)((int)pvVar1 + 4) == -1) {
    local_14[1] = 0x12;
    local_14[0] = 1;
    local_4 = 1;
    uVar4 = FUN_00504e60((uint *)((int)this + 0x3c),&local_18,local_14,pvVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if (uVar4 != 0) {
    if (*(int *)((int)pvVar1 + 4) == -1) {
      uVar4 = (**(code **)(*local_18 + 0x6c))((int)this + 0x40,pvVar1);
    }
    if ((uVar4 != 0) && (*(int *)((int)pvVar1 + 4) == -1)) {
      puVar3 = FUN_004025b0(local_18,(uint *)&param_1);
      local_4 = 2;
      bVar2 = FUN_005152e0(puVar3,pvVar1);
      uVar4 = CONCAT31(extraout_var,bVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return uVar4;
}

