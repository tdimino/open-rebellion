
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005c30b0(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  int unaff_retaddr;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653b11;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f4d90(param_1,(float *)((int)this + 0x624));
  FUN_005f4d90(param_1,(int)this + 0x648);
  local_10 = this;
  if (*(int *)((int)this + 0x63c) == 0) {
    local_10 = (void *)FUN_00618b70(0x50);
    local_4 = 0;
    if (local_10 == (void *)0x0) {
      local_4 = 0xffffffff;
      *(undefined4 *)((int)this + 0x63c) = 0;
    }
    else {
      pvVar1 = FUN_005d9490(local_10,_DAT_0066c958 * *(float *)((int)this + 0x624),this,
                            *(int **)((int)this + 0x128),*(int **)((int)this + 0x130),
                            *(undefined4 *)((int)this + 0x138),0,0,0,0);
      *(void **)((int)this + 0x63c) = pvVar1;
      local_4 = 0xffffffff;
    }
  }
  (**(code **)(**(int **)((int)this + 0x63c) + 4))(param_1);
  FUN_005f4d90(param_1,&stack0x00000000);
  if (unaff_retaddr != 0) {
    if (*(int *)((int)this + 0x640) == 0) {
      pvVar1 = (void *)FUN_00618b70(0x50);
      puStack_8 = (undefined1 *)0x1;
      if (pvVar1 == (void *)0x0) {
        pvVar1 = (void *)0x0;
      }
      else {
        pvVar1 = FUN_005d9580(pvVar1,*(int *)((int)this + 0x63c));
      }
      puStack_8 = (undefined1 *)0xffffffff;
      *(void **)((int)this + 0x640) = pvVar1;
    }
    (**(code **)(**(int **)((int)this + 0x640) + 4))(param_1);
  }
  FUN_005f4d90(param_1,&stack0x00000000);
  if (unaff_retaddr != 0) {
    if (*(int *)((int)this + 0x644) == 0) {
      pvVar1 = (void *)FUN_00618b70(0x3c);
      puStack_8 = (undefined1 *)0x2;
      if (pvVar1 == (void *)0x0) {
        pvVar1 = (void *)0x0;
      }
      else {
        pvVar1 = FUN_005d9a70(pvVar1,0x3e4ccccd,1,*(int *)((int)this + 0x63c));
      }
      puStack_8 = (undefined1 *)0xffffffff;
      *(void **)((int)this + 0x644) = pvVar1;
    }
    (**(code **)(**(int **)((int)this + 0x644) + 4))(param_1);
  }
  *(undefined4 *)((int)this + 0x638) = 0;
  if (*(int *)((int)this + 0x644) == 0) {
    iVar2 = *(int *)((int)this + 0x63c);
    if (iVar2 == 0) {
      ExceptionList = local_10;
      return;
    }
  }
  else {
    iVar2 = FUN_005d9ae0(*(int *)((int)this + 0x644));
    if (iVar2 == 0) {
      *(undefined4 *)((int)this + 0x638) = *(undefined4 *)((int)this + 0x63c);
      if (*(undefined4 **)((int)this + 0x644) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)((int)this + 0x644))(1);
      }
      *(undefined4 *)((int)this + 0x644) = 0;
      ExceptionList = local_10;
      return;
    }
    iVar2 = *(int *)((int)this + 0x644);
  }
  *(int *)((int)this + 0x638) = iVar2;
  ExceptionList = local_10;
  return;
}

