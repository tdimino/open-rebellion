
void * __thiscall
FUN_005d6700(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,undefined4 param_7,undefined4 param_8,undefined4 param_9,
            uint param_10,undefined4 param_11,undefined4 param_12,undefined4 param_13)

{
  HCURSOR pHVar1;
  HGDIOBJ pvVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  uint uVar6;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006549f4;
  pvStack_c = ExceptionList;
  uVar6 = 0;
  uVar5 = 0;
  uVar4 = 0;
  ExceptionList = &pvStack_c;
  pHVar1 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f00);
  uVar3 = 0;
  pvVar2 = GetStockObject(4);
  FUN_005ff910(this,param_1,s_3D_WINDOW_006abea8,param_10 | 0x52000000,param_2,param_3,param_4,
               param_5,param_6,(char *)0x0,pvVar2,uVar3,(int)pHVar1,uVar4,uVar5,uVar6);
  uStack_4 = 0;
  FUN_005eec90((undefined4 *)((int)this + 0x100));
  uStack_4._0_1_ = 1;
  FUN_005eed20((undefined4 *)((int)this + 0x10c));
  uStack_4 = CONCAT31(uStack_4._1_3_,2);
  FUN_005f66c0((undefined4 *)((int)this + 0x118));
  *(undefined4 *)((int)this + 0xf4) = param_7;
  *(undefined4 *)((int)this + 0xf8) = param_8;
  *(undefined4 *)((int)this + 0xfc) = param_9;
  *(undefined ***)this = &PTR_FUN_0066d2e0;
  *(undefined4 *)((int)this + 0x14c) = 0;
  *(undefined4 *)((int)this + 0x150) = 0;
  *(undefined4 *)((int)this + 0x144) = param_11;
  *(undefined4 *)((int)this + 0x120) = param_12;
  *(undefined4 *)((int)this + 0x148) = param_13;
  DAT_006be6fc = 1;
  *(undefined4 *)((int)this + 0x14c) = DAT_006be6d4;
  uVar3 = DAT_006be6d8;
  *(undefined4 *)((int)this + 0x164) = 0;
  *(undefined4 *)((int)this + 0x160) = 0;
  *(undefined4 *)((int)this + 0x15c) = 0;
  *(undefined4 *)((int)this + 0x158) = 0;
  *(undefined4 *)((int)this + 0x154) = 0;
  *(undefined4 *)((int)this + 0xec) = 0;
  *(undefined4 *)((int)this + 0x150) = uVar3;
  ExceptionList = pvStack_c;
  return this;
}

