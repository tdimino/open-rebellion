
bool __cdecl FUN_005150c0(uint *param_1,undefined4 param_2,void *param_3)

{
  uint *puVar1;
  bool bVar2;
  int *this;
  uint uVar3;
  int iVar4;
  void *pvVar5;
  undefined3 extraout_var;
  uint *puVar6;
  bool bVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642658;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (int *)FUN_00504dc0(param_1);
  bVar7 = this != (int *)0x0;
  if (this != (int *)0x0) {
    local_4 = 0;
    uVar3 = (**(code **)(*this + 4))();
    if ((uVar3 < 0x10) || (0x3f < uVar3)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar2) {
      puVar1 = (uint *)(this + 0xe);
      iVar4 = FUN_004ece60(puVar1);
      if (iVar4 != 0) {
        pvVar5 = (void *)FUN_00505750(puVar1);
        if ((pvVar5 == (void *)0x0) || (!bVar7)) {
          bVar7 = false;
        }
        else {
          bVar7 = true;
        }
        if (pvVar5 != (void *)0x0) {
          iVar4 = FUN_00529d70(pvVar5,param_1,param_2,param_3);
          if ((iVar4 == 0) || (!bVar7)) {
            bVar7 = false;
          }
          else {
            bVar7 = true;
          }
        }
      }
      bVar2 = true;
      local_4 = 1;
      uVar3 = (**(code **)(*this + 4))();
      if ((uVar3 < 0x30) || (0x3f < uVar3)) {
        bVar2 = false;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      if (bVar2) {
        pvVar5 = (void *)thunk_FUN_00506e40();
        if ((pvVar5 == (void *)0x0) || (!bVar7)) {
          bVar7 = false;
        }
        else {
          bVar7 = true;
        }
        if (pvVar5 == (void *)0x0) {
          ExceptionList = local_c;
          return bVar7;
        }
        bVar2 = FUN_00545180(pvVar5,param_1,param_2,(uint)param_3);
        if ((CONCAT31(extraout_var,bVar2) != 0) && (bVar7)) {
          ExceptionList = local_c;
          return true;
        }
      }
      else {
        iVar4 = thunk_FUN_00506e60();
        if ((iVar4 == 0) || (!bVar7)) {
          bVar7 = false;
        }
        else {
          bVar7 = true;
        }
        if ((iVar4 != 0) && (iVar4 = FUN_004ece60(puVar1), iVar4 != 0)) {
          puVar6 = FUN_004025b0(this,(uint *)&param_1);
          local_4 = 2;
          uVar3 = FUN_00556430(this,puVar1,puVar6,param_3);
          if ((uVar3 == 0) || (!bVar7)) {
            bVar7 = false;
          }
          else {
            bVar7 = true;
          }
          local_4 = 0xffffffff;
          FUN_00619730();
        }
        if ((*(byte *)(this + 0x14) & 0x20) != 0) {
          ExceptionList = local_c;
          return bVar7;
        }
        uVar3 = FUN_00556710(this,param_3);
        if ((uVar3 != 0) && (bVar7)) {
          ExceptionList = local_c;
          return true;
        }
      }
      bVar7 = false;
    }
  }
  ExceptionList = local_c;
  return bVar7;
}

