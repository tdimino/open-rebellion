
void __thiscall FUN_00452240(void *this,HDC param_1)

{
  FUN_005fcc30(*(void **)(*(int *)((int)this + 0x11c) + 0x45c),*(uint **)((int)this + 0xa4),0,0,
               *(int *)((int)this + 0x28),*(uint *)((int)this + 0x2c),
               *(undefined4 **)((int)this + 0x38),*(uint *)((int)this + 0x3c));
  FUN_005fe050(*(void **)((int)this + 0xa4),0,0,(short)*(undefined4 *)((int)this + 0x38),
               (short)*(undefined4 *)((int)this + 0x3c));
  if (*(void **)((int)this + 0x148) != (void *)0x0) {
    FUN_005fd0f0(*(void **)((int)this + 0x148),*(uint **)((int)this + 0xa4),0,0);
  }
  if (*(void **)((int)this + 0xa8) != (void *)0x0) {
    FUN_005fd0f0(*(void **)((int)this + 0xa8),*(uint **)((int)this + 0xa4),0,0);
  }
  FUN_005fc140(*(void **)((int)this + 0xa4),param_1,'@',0xcc0020,0,0,0,0,0,0);
  return;
}

