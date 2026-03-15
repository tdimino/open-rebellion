
undefined4 __thiscall FUN_005072d0(void *this,int *param_1,void *param_2)

{
  bool bVar1;
  uint uVar2;
  undefined4 uVar3;
  uint local_14;
  uint local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006415f0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (((*(byte *)((int)this + 0x88) & 1) == 0) &&
     (ExceptionList = &pvStack_c, ((byte)*(undefined4 *)((int)this + 0x24) & 0xc0) == 0xc0)) {
    local_14 = 0x10;
    local_10 = 0x14;
    bVar1 = false;
    local_4 = 0;
    ExceptionList = &pvStack_c;
    uVar2 = (**(code **)(*param_1 + 4))();
    if ((local_14 <= uVar2) && (uVar2 < local_10)) {
      bVar1 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      local_10 = 0xffffffff;
      local_14 = 1;
      local_4 = 1;
      FUN_00520580(param_2,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = pvStack_c;
      return 1;
    }
  }
  uVar3 = FUN_004f6490(this,(int)param_1,param_2);
  ExceptionList = pvStack_c;
  return uVar3;
}

