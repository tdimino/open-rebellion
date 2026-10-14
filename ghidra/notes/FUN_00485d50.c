
void __thiscall FUN_00485d50(void *this,undefined4 param_1,undefined4 *param_2)

{
  void *this_00;
  
  FUN_004fcdd0((undefined4 *)((int)this + 0x3c));
  if (*(int *)((int)this + 0x418) == 0) {
    this_00 = (void *)FUN_005f5500((void *)((int)this + 100),0xd);
    if (this_00 != (void *)0x0) {
      FUN_004bf300(this_00,param_2,(undefined4 *)((int)this + 0x3c));
    }
  }
  *(undefined4 *)((int)this + 0x44) = param_1;
  FUN_004f26d0((void *)((int)this + 0x40),param_2);
  *(undefined4 *)((int)this + 0x38) = 1;
  return;
}


