
void __thiscall FUN_0060d7e0(void *this,uint param_1,int param_2)

{
  bool bVar1;
  void *this_00;
  undefined3 extraout_var;
  
  this_00 = (void *)FUN_00604500((void *)((int)this + 0x6c),param_1);
  if (this_00 != (void *)0x0) {
    if (*(void **)((int)this + 0x94) != (void *)0x0) {
      FUN_006030f0(*(void **)((int)this + 0x94),4);
    }
    bVar1 = FUN_00603120(this_00,4);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      FUN_006030c0(this_00,4);
    }
    *(void **)((int)this + 0x94) = this_00;
    if (param_2 != 0) {
      SendMessageA(*(HWND *)(*(int *)((int)this + 0x20) + 0x18),0x111,
                   *(int *)((int)this_00 + 0x24) << 0x10 | *(uint *)((int)this + 0x24) & 0xffff,0);
    }
  }
  return;
}

