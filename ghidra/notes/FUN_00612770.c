
void * __thiscall FUN_00612770(void *this,int *param_1)

{
  void *this_00;
  void *pvVar1;
  void *pvVar2;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656f86;
  local_c = ExceptionList;
  this_00 = (void *)((int)this + 0x40);
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00612bd0(this_00,1);
  if (pvVar1 == (void *)0x0) {
    uVar3 = (uint)(*(int *)((int)this + 0x2c) * 2000) / 1000;
    if ((*param_1 == 2) || (*(uint *)((int)this + 0x38) < uVar3)) {
      uVar3 = *(uint *)((int)this + 0x38);
      *param_1 = 2;
    }
    pvVar1 = (void *)FUN_00618b70(0xc4);
    local_4 = 0;
    if (pvVar1 == (void *)0x0) {
      pvVar1 = (void *)0x0;
    }
    else {
      pvVar1 = FUN_00613360(pvVar1,*(undefined4 *)((int)this + 0x18),(int)this + 0x24,uVar3,*param_1
                            ,*(undefined4 *)((int)this + 0x38));
    }
    local_4 = 0xffffffff;
    if (pvVar1 != (void *)0x0) {
      *(undefined4 *)((int)pvVar1 + 0x18) = 1;
      FUN_00612b30(this_00,(int)pvVar1);
    }
  }
  else {
    pvVar2 = (void *)FUN_00618b70(0xc4);
    local_4 = 1;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_006134d0(pvVar2,(int)pvVar1);
    }
    local_4 = 0xffffffff;
    if (pvVar2 != (void *)0x0) {
      FUN_00612b30(this_00,(int)pvVar2);
      pvVar1 = pvVar2;
    }
  }
  ExceptionList = local_c;
  return pvVar1;
}

