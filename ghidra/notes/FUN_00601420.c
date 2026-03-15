
bool __cdecl FUN_00601420(uint param_1)

{
  int iVar1;
  void *pvVar2;
  uint uVar3;
  bool bVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065610b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar1 = FUN_005f5500(&DAT_006be4e8,param_1);
  bVar4 = iVar1 == 0;
  if (bVar4) {
    bVar4 = false;
    pvVar2 = (void *)FUN_00618b70(0x1c);
    local_4 = 0;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_005f55d0(pvVar2,param_1);
    }
    local_4 = 0xffffffff;
    if (pvVar2 != (void *)0x0) {
      uVar3 = FUN_005f5440(&DAT_006be4e8,pvVar2);
      bVar4 = uVar3 == param_1;
    }
  }
  ExceptionList = local_c;
  return bVar4;
}

