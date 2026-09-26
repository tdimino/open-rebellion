
bool __cdecl FUN_00515a50(uint *param_1,int param_2,void *param_3)

{
  uint *puVar1;
  void *pvVar2;
  bool bVar3;
  bool bVar4;
  int *this;
  uint uVar5;
  int iVar6;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642748;
  local_c = ExceptionList;
  bVar3 = true;
  ExceptionList = &local_c;
  this = (int *)FUN_00504dc0(param_1);
  if ((this != (int *)0x0) && ((*(byte *)(this + 0x14) & 8) == 0)) {
    local_4 = 0;
    uVar5 = (**(code **)(*this + 4))();
    if ((uVar5 < 0x30) || (0x3f < uVar5)) {
      bVar3 = false;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar3) {
      iVar6 = thunk_FUN_00506e40();
      if (iVar6 == 0) {
        ExceptionList = local_c;
        return false;
      }
      bVar3 = FUN_00545820(puVar1,param_2,param_3);
      if ((CONCAT31(extraout_var,bVar3) != 0) && (iVar6 != 0)) {
        ExceptionList = local_c;
        return true;
      }
    }
    else {
      iVar6 = thunk_FUN_00506e60();
      pvVar2 = param_3;
      bVar3 = iVar6 != 0;
      param_1 = (uint *)0x1;
      if (iVar6 != 0) {
        bVar4 = FUN_00556620(this,(uint *)&param_1);
        if ((CONCAT31(extraout_var_00,bVar4) == 0) || (!bVar3)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
      }
      if (param_1 == (uint *)0x0) {
        ExceptionList = local_c;
        return bVar3;
      }
      iVar6 = FUN_004f7640(this,0,pvVar2);
      if ((iVar6 != 0) && (bVar3)) {
        ExceptionList = local_c;
        return true;
      }
    }
    bVar3 = false;
  }
  ExceptionList = local_c;
  return bVar3;
}

