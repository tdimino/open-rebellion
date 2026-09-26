
uint __thiscall FUN_00530350(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  void *this_00;
  uint uVar3;
  bool bVar4;
  int local_c;
  int local_8;
  int local_4;
  
  this_00 = (void *)0x0;
  local_c = 0;
  if ((*(int *)((int)this + 0x58) < *(int *)((int)this + 0x70)) ||
     (*(int *)((int)this + 0x58) < *(int *)((int)this + 0x74))) {
    uVar3 = 1;
  }
  else {
    uVar3 = 0;
  }
  for (pvVar1 = (void *)FUN_00505340(); pvVar1 != (void *)0x0; pvVar1 = (void *)FUN_0052e8b0(pvVar1)
      ) {
    if (uVar3 == 0) {
      return 0;
    }
    uVar3 = FUN_0052e530(this,(int)pvVar1,&local_8);
    if ((uVar3 != 0) && (local_8 != 0)) {
      local_c = local_c + 1;
    }
  }
  if ((uVar3 != 0) && (local_c != 0)) {
    local_4 = 0;
    iVar2 = FUN_0053e290(local_c + -1);
    for (pvVar1 = (void *)FUN_00505340(); pvVar1 != (void *)0x0;
        pvVar1 = (void *)FUN_0052e8b0(pvVar1)) {
      if (uVar3 == 0) {
        return 0;
      }
      if (this_00 != (void *)0x0) break;
      uVar3 = FUN_0052e530(this,(int)pvVar1,&local_8);
      if ((uVar3 != 0) && (local_8 != 0)) {
        if (local_4 == iVar2) {
          this_00 = pvVar1;
        }
        local_4 = local_4 + 1;
      }
    }
    bVar4 = uVar3 != 0;
    uVar3 = 0;
    if ((bVar4) && (uVar3 = 0, this_00 != (void *)0x0)) {
      uVar3 = FUN_004f7950(this_00,1,param_1);
    }
  }
  return uVar3;
}

