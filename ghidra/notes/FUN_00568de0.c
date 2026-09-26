
void * __thiscall FUN_00568de0(void *this,void *param_1)

{
  void *this_00;
  bool bVar1;
  void *pvVar2;
  void *pvVar3;
  undefined4 *puVar4;
  int iVar5;
  uint local_34 [5];
  undefined1 local_20 [20];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064b3d0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00568fe0(local_34);
  this_00 = *(void **)((int)this + 4);
  local_4 = 0;
  pvVar3 = (void *)0x0;
  do {
    pvVar2 = pvVar3;
    if ((this_00 == (void *)0x0) || (pvVar2 != (void *)0x0)) {
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = local_c;
      return pvVar2;
    }
    puVar4 = FUN_00568ec0(this_00,local_20);
    local_4._0_1_ = 1;
    FUN_00569030(local_34,puVar4);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    iVar5 = FUN_005690b0(param_1,local_34);
    if (iVar5 == 0) {
      iVar5 = FUN_00569060(param_1,(int *)local_34);
      bVar1 = false;
      if (iVar5 != 0) goto LAB_00568e66;
    }
    else {
LAB_00568e66:
      bVar1 = true;
    }
    if (bVar1) {
      iVar5 = FUN_005690b0(param_1,local_34);
      pvVar3 = this_00;
      if (iVar5 != 0) {
        this_00 = *(void **)((int)this_00 + 4);
        pvVar3 = pvVar2;
      }
    }
    else {
      this_00 = *(void **)((int)this_00 + 8);
      pvVar3 = pvVar2;
    }
  } while( true );
}

