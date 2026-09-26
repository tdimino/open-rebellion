
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005c1d30(void *this,undefined4 param_1,float param_2)

{
  void *pvVar1;
  int iVar2;
  undefined4 uVar3;
  undefined4 local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653a56;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(float *)((int)this + 0x624) = param_2;
  pvVar1 = (void *)FUN_00618b70(0x70);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005d4d10(pvVar1,*(int **)((int)this + 0x138),param_2,param_1);
  }
  local_4 = 0xffffffff;
  *(void **)((int)this + 0x628) = pvVar1;
  FUN_005f5440((void *)((int)this + 0x5d8),pvVar1);
  if (*(int *)((int)this + 0x63c) == 0) {
    pvVar1 = (void *)FUN_00618b70(0x50);
    local_4 = 1;
    if (pvVar1 == (void *)0x0) {
      pvVar1 = (void *)0x0;
    }
    else {
      pvVar1 = FUN_005d9490(pvVar1,param_2 * _DAT_0066c958,this,*(int **)((int)this + 0x128),
                            *(int **)((int)this + 0x130),*(undefined4 *)((int)this + 0x138),0,0,0,0)
      ;
    }
    *(void **)((int)this + 0x63c) = pvVar1;
    local_4 = 0xffffffff;
    FUN_00595d20(local_1c,1);
    FUN_00595d20(local_1c,0);
    iVar2 = FUN_00595c40();
    if (iVar2 == 1) {
      uVar3 = 0x96;
    }
    else {
      uVar3 = 0xffffffe2;
    }
    FUN_005d9620(*(void **)((int)this + 0x63c),0x1e,uVar3,0x3e4ccccd);
  }
  ExceptionList = local_c;
  return;
}

