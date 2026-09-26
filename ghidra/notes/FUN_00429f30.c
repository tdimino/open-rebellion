
void __thiscall FUN_00429f30(void *this,undefined4 *param_1,undefined4 *param_2)

{
  int iVar1;
  void *this_00;
  int *this_01;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062c47b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  PostMessageA(*(HWND *)((int)this + 0x18),0x409,0,0);
  iVar1 = FUN_00604500((void *)((int)this + 0x6c),0x19);
  if (iVar1 == 0) {
    this_00 = (void *)FUN_00618b70(0x164);
    uStack_4 = 0;
    if (this_00 == (void *)0x0) {
      this_01 = (int *)0x0;
    }
    else {
      this_01 = FUN_0045d400(this_00,*(undefined4 *)((int)this + 0x1c),0,(undefined4 *)0x0,0x1d6,
                             (int *)0x14a,(uint)this,(int *)0x19,param_1,param_2);
    }
    uStack_4 = 0xffffffff;
    if (this_01 != (int *)0x0) {
      this_01[0x14] = this_01[0x14] & 0xefffffff;
      FUN_005ffce0(this_01,0);
      FUN_005f4f10((void *)((int)this + 0x6c),(int)this_01);
      (**(code **)(*this_01 + 0x2c))(5);
    }
  }
  ExceptionList = pvStack_c;
  return;
}

