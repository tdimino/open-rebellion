
uint FUN_0055f4c0(uint *param_1,void *param_2,void *param_3)

{
  void *this;
  int *piVar1;
  uint uVar2;
  uint *puVar3;
  int *piVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  bool bVar8;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_3;
  puStack_8 = &LAB_0064a010;
  local_c = ExceptionList;
  uVar6 = 1;
  local_14 = 1;
  local_10 = 1;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar1 = (int *)FUN_00504dc0(param_1);
  if (piVar1 != (int *)0x0) {
    local_14 = 0x30;
    local_10 = 0x3c;
    local_4 = 1;
    uVar2 = (**(code **)(*piVar1 + 4))();
    if ((uVar2 < local_14) || (local_10 <= uVar2)) {
      uVar6 = 0;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    bVar8 = uVar6 != 0;
    uVar6 = 0;
    if (bVar8) {
      uVar6 = (**(code **)(*piVar1 + 0x2dc))(param_2,this);
    }
    if ((uVar6 != 0) && (*(int *)((int)this + 4) == -1)) {
      uVar6 = (uint)piVar1[9] >> 6 & 3;
      if (uVar6 == 1) {
        iVar7 = 1;
      }
      else if (uVar6 == 2) {
        iVar7 = 2;
      }
      else {
        iVar7 = 0;
      }
      puVar3 = FUN_00562ac0(param_2,&param_3);
      local_4 = 2;
      piVar4 = FUN_004f2ec0(iVar7,puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      uVar6 = (uint)(piVar4 != (int *)0x0);
      if (uVar6 != 0) {
        uVar2 = piVar1[9];
        uVar5 = FUN_004edc80((int)piVar4);
        if (uVar5 != (uVar2 >> 6 & 3)) {
          local_14 = 1;
          local_10 = 0x24;
          local_4 = 3;
          FUN_00520580(this,&local_14);
          local_4 = 0xffffffff;
          FUN_00619730();
        }
      }
    }
  }
  ExceptionList = local_c;
  return uVar6;
}

