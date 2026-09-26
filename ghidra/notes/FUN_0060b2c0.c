
void __thiscall FUN_0060b2c0(void *this,uint *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  void *pvVar3;
  tagPOINT local_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065699b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_005fefd0((uint)*(ushort *)((int)this + 0xe8));
  if ((param_1 != (uint *)0x0) && (iVar2 != 0)) {
    puVar1 = *(undefined4 **)((int)this + 200);
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
    }
    pvVar3 = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005fbda0(pvVar3,*(int *)((int)this + 0x30),*(int *)((int)this + 0x34),param_1);
    }
    local_4 = 0xffffffff;
    *(void **)((int)this + 200) = pvVar3;
    if (pvVar3 != (void *)0x0) {
      local_14.x = 0;
      local_14.y = 0;
      if (*(int *)((int)this + 0x20) != 0) {
        MapWindowPoints(*(HWND *)(*(int *)((int)this + 0x20) + 0x18),*(HWND *)((int)this + 0x18),
                        &local_14,1);
        FUN_005fcc30(param_1,*(uint **)((int)this + 200),0,0,local_14.x,local_14.y,
                     *(undefined4 **)((int)this + 0x30),*(uint *)((int)this + 0x34));
        FUN_0060b4b0(this);
      }
    }
  }
  ExceptionList = local_c;
  return;
}

