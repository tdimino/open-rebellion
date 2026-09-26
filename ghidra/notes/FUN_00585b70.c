
uint __thiscall FUN_00585b70(void *this,int *param_1)

{
  int *piVar1;
  uint *puVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  uint uVar5;
  bool bVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f587;
  local_c = ExceptionList;
  bVar6 = param_1 != (int *)0x0;
  if (bVar6) {
    ExceptionList = &local_c;
    puVar2 = FUN_004025b0(param_1,(uint *)&param_1);
    local_4 = 0;
    puVar3 = FUN_00585dc0(this,puVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (puVar3 == (undefined4 *)0x0) {
      puVar3 = (undefined4 *)FUN_00618b70(0x30);
      if (puVar3 == (undefined4 *)0x0) {
        FUN_00619730();
      }
      local_4 = 1;
      if (puVar3 == (undefined4 *)0x0) {
        puVar3 = (undefined4 *)0x0;
      }
      else {
        puVar3 = FUN_00584370(puVar3);
      }
      local_4 = 0xffffffff;
      bVar6 = puVar3 != (undefined4 *)0x0;
      if (!bVar6) {
        ExceptionList = local_c;
        return 0;
      }
      FUN_005f5600(puVar3,piVar1[6]);
      uVar4 = (**(code **)(*piVar1 + 4))();
      puVar3[8] = uVar4;
      FUN_005f5440(this,puVar3);
    }
  }
  else {
    puVar3 = (undefined4 *)0x0;
    ExceptionList = &local_c;
  }
  uVar5 = 0;
  if (bVar6) {
    uVar5 = FUN_00584410(puVar3,piVar1);
  }
  ExceptionList = local_c;
  return uVar5;
}

