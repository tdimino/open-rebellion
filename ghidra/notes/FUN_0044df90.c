
void __thiscall FUN_0044df90(int *param_1,uint param_2,uint param_3)

{
  uint uVar1;
  int *this;
  uint *puVar2;
  void *this_00;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uVar1 = param_2;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062f830;
  pvStack_c = ExceptionList;
  switch(param_2 & 0xffff) {
  case 10:
    ExceptionList = &pvStack_c;
    (**(code **)(*param_1 + 0x30))();
    FUN_00469d60(param_1[0x2b],*(int *)(param_1[0x5d] + 0x18));
    break;
  case 0xb:
  case 0xc:
    ExceptionList = &pvStack_c;
    this = FUN_004f2d10(*(int *)(param_1[0x46] + 0x9c),(uint *)(param_1[0x5d] + 0x5c));
    if (this != (int *)0x0) {
      puVar2 = FUN_004025b0((void *)this[7],&param_3);
      uStack_4 = 0;
      this_00 = FUN_00429ce0((void *)param_1[0x46],puVar2);
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if ((short)uVar1 == 0xc) {
        puVar2 = FUN_004025b0(this,&param_2);
        uStack_4 = 1;
        FUN_0045c8e0(this_00,(ushort *)puVar2,4);
        uStack_4 = 0xffffffff;
        FUN_00619730();
      }
    }
    FUN_00469db0(param_1[0x2b]);
    (**(code **)(*param_1 + 0x30))();
    break;
  case 100:
    ExceptionList = &pvStack_c;
    FUN_0044e3d0(param_1,param_3 >> 0x10);
    break;
  case 0x96:
    ExceptionList = &pvStack_c;
    FUN_0044e500(param_1,param_3 >> 0x10,0);
  }
  ExceptionList = pvStack_c;
  return;
}

