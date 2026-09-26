
void * __thiscall FUN_004861b0(void *this,int param_1)

{
  undefined4 *puVar1;
  undefined1 uVar2;
  void *pvVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006348e4;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004884f0(this,param_1);
  local_4 = 0;
  FUN_004bee50((undefined4 *)((int)this + 0x5c));
  puVar1 = (undefined4 *)((int)this + 0x6c);
  local_4._0_1_ = 1;
  FUN_004888f0(puVar1);
  local_4._0_1_ = 2;
  uVar2 = (undefined1)local_4;
  local_4._0_1_ = 2;
  *(undefined ***)this = &PTR_FUN_0065ad70;
  *(undefined4 *)((int)this + 0x50) = 0;
  *(undefined4 *)((int)this + 0x54) = 0;
  *(undefined4 *)((int)this + 0xc0) = 0;
  *(undefined4 *)((int)this + 0x58) = 0;
  if (*(int *)((int)this + 0x24) == 1) {
    pvVar3 = (void *)FUN_00618b70(0x194);
    local_4 = CONCAT31(local_4._1_3_,3);
    if (pvVar3 == (void *)0x0) {
LAB_0048626d:
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_004c27f0(pvVar3,*(undefined4 *)((int)this + 0x24),this);
    }
  }
  else {
    if (*(int *)((int)this + 0x24) != 2) goto LAB_00486275;
    local_4._0_1_ = uVar2;
    pvVar3 = (void *)FUN_00618b70(0x194);
    local_4 = CONCAT31(local_4._1_3_,4);
    if (pvVar3 == (void *)0x0) goto LAB_0048626d;
    pvVar3 = FUN_004c0710(pvVar3,*(undefined4 *)((int)this + 0x24),this);
  }
  *(void **)((int)this + 0xc0) = pvVar3;
LAB_00486275:
  *(undefined4 **)(*(int *)((int)this + 0xc0) + 8) = puVar1;
  *(undefined4 *)((int)this + 0x70) = *(undefined4 *)((int)this + 0xc0);
  *puVar1 = *(undefined4 *)((int)this + 0x24);
  ExceptionList = local_c;
  return this;
}

