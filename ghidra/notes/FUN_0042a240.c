
void __thiscall FUN_0042a240(void *this,undefined4 param_1,undefined4 param_2)

{
  int iVar1;
  uint uVar2;
  void *this_00;
  int *this_01;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062c4fb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  PostMessageA(*(HWND *)((int)this + 0x18),0x409,0,0);
  iVar1 = FUN_00604500((void *)((int)this + 0x6c),0xd);
  if (iVar1 == 0) {
    uVar2 = FUN_004fcee0();
    if (1 < uVar2) {
      this_00 = (void *)FUN_00618b70(0x1e0);
      uStack_4 = 0;
      if (this_00 == (void *)0x0) {
        this_01 = (int *)0x0;
      }
      else {
        this_01 = FUN_00466350(this_00,*(undefined4 *)((int)this + 0x1c),(int)this,0xd,param_1,
                               param_2);
      }
      uStack_4 = 0xffffffff;
      if (this_01 != (int *)0x0) {
        FUN_005ffce0(this_01,0);
        FUN_005f4f10((void *)((int)this + 0x6c),(int)this_01);
        (**(code **)(*this_01 + 0x2c))(5);
      }
    }
  }
  ExceptionList = pvStack_c;
  return;
}

