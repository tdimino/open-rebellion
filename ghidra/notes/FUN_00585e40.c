
void * __thiscall FUN_00585e40(void *this,uint *param_1)

{
  uint uVar1;
  bool bVar2;
  void **ppvVar3;
  uint *puVar4;
  uint uVar5;
  void *pvVar6;
  void *this_00;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064f5c8;
  uVar5 = *param_1 << 0x18 | 1;
  bVar2 = false;
  local_4 = 0;
  ppvVar3 = &local_c;
  pvVar6 = (void *)0x0;
  this_00 = *(void **)((int)this + 4);
  local_c = ExceptionList;
  while ((ExceptionList = ppvVar3, this_00 != (void *)0x0 && (!bVar2))) {
    puVar4 = (uint *)FUN_005843d0(this_00,&local_14);
    uVar1 = *puVar4;
    FUN_00619730();
    bVar2 = false;
    pvVar6 = this_00;
    if (uVar1 < uVar5) {
      ppvVar3 = ExceptionList;
      this_00 = *(void **)((int)this_00 + 8);
    }
    else {
      puVar4 = (uint *)FUN_005843d0(this_00,&local_10);
      uVar1 = *puVar4;
      FUN_00619730();
      ppvVar3 = ExceptionList;
      if (uVar5 < uVar1) {
        this_00 = *(void **)((int)this_00 + 4);
      }
      else {
        bVar2 = true;
      }
    }
  }
  if ((this_00 == (void *)0x0) && (pvVar6 != (void *)0x0)) {
    puVar4 = (uint *)FUN_005843d0(pvVar6,&local_10);
    uVar1 = *puVar4;
    FUN_00619730();
    if (uVar1 < uVar5) {
      pvVar6 = *(void **)((int)pvVar6 + 0x10);
    }
    if (((pvVar6 != (void *)0x0) && (*param_1 <= *(uint *)((int)pvVar6 + 0x20))) &&
       (*(uint *)((int)pvVar6 + 0x20) < param_1[1])) {
      this_00 = pvVar6;
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return this_00;
}

