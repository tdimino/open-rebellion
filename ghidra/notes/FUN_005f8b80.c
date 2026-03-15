
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __thiscall
FUN_005f8b80(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5,uint param_6)

{
  DWORD DVar1;
  uint uVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  undefined4 auStack_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00655bd0;
  pvStack_c = ExceptionList;
  uVar4 = 0;
  DAT_006be3a4 = 0;
  ExceptionList = &pvStack_c;
  DVar1 = timeGetTime();
  *(DWORD *)((int)this + 0xd0) = DVar1;
  *(undefined4 *)((int)this + 0xd4) = 0;
  *(undefined4 *)((int)this + 0xf8) = 0;
  *(undefined4 *)((int)this + 0xfc) = 0;
  *(undefined4 *)((int)this + 0x100) = 0;
  *(undefined4 *)((int)this + 0x104) = 0;
  FUN_005f35b0(auStack_18,s_MinimumSendLatency_006a82d0);
  uStack_4 = 0;
  FUN_005f47c0(&DAT_006be370,(int)auStack_18,(undefined4 *)((int)this + 0xd8));
  uStack_4 = 0xffffffff;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_MaximumSendLatency_006a82bc);
  uStack_4 = 1;
  FUN_005f47c0(&DAT_006be370,(int)auStack_18,(undefined4 *)((int)this + 0xdc));
  uStack_4 = 0xffffffff;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_MinimumReceiveLatency_006a82fc);
  uStack_4 = 2;
  FUN_005f47c0(&DAT_006be370,(int)auStack_18,(undefined4 *)((int)this + 0xe0));
  uStack_4 = 0xffffffff;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_MaximumReceiveLatency_006a82e4);
  uStack_4 = 3;
  FUN_005f47c0(&DAT_006be370,(int)auStack_18,(undefined4 *)((int)this + 0xe4));
  uStack_4 = 0xffffffff;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_DropSendPercent_006a8298);
  uStack_4 = 4;
  FUN_005f4770(&DAT_006be370,(uint)auStack_18,(float *)((int)this + 0xe8));
  uStack_4 = 0xffffffff;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_DropReceivePercent_006a82a8);
  uStack_4 = 5;
  FUN_005f4770(&DAT_006be370,(uint)auStack_18,(float *)((int)this + 0xec));
  uStack_4 = 0xffffffff;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_DupeSendPercent_006a8274);
  uStack_4 = 6;
  FUN_005f4770(&DAT_006be370,(uint)auStack_18,(float *)((int)this + 0xf0));
  uStack_4 = 0xffffffff;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_DupeReceivePercent_006a8284);
  uStack_4 = 7;
  FUN_005f4770(&DAT_006be370,(uint)auStack_18,(float *)((int)this + 0xf4));
  uStack_4 = 0xffffffff;
  FUN_005f2ff0(auStack_18);
  *(undefined4 *)((int)this + 0x2c) = param_1;
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined4 *)((int)this + 0xb0) = param_2;
  *(undefined4 *)((int)this + 0x28) = 1;
  *(undefined4 *)((int)this + 0xb4) = param_3;
  *(undefined4 *)((int)this + 0xb8) = param_4;
  *(undefined4 *)((int)this + 0xbc) = param_5;
  (*(code *)**(undefined4 **)((int)this + 0x247c))();
  _DAT_006be3ac = 0;
  *(undefined4 *)((int)this + 0x108) = 0;
  *(undefined4 *)((int)this + 0x10c) = 0xffffffff;
  *(undefined4 *)((int)this + 0x94) = 0;
  *(undefined4 *)((int)this + 0x90) = 0;
  *(undefined4 *)((int)this + 0xa4) = 0;
  *(undefined4 *)((int)this + 0xa8) = 0x3000;
  DVar1 = GetTickCount();
  *(DWORD *)((int)this + 0xac) = DVar1 + *(int *)((int)this + 0xa8);
  DAT_006ac294 = param_6;
  *(undefined4 *)((int)this + 0x194) = 1000;
  uVar2 = 0;
  if (DAT_006ac294 != 0) {
    puVar3 = (undefined4 *)((int)this + 0x34);
    do {
      *puVar3 = 0xffffffff;
      uVar2 = uVar2 + 1;
      puVar3 = puVar3 + 1;
    } while (uVar2 < DAT_006ac294);
  }
  if (DAT_006be354 != 0) {
    *(undefined4 *)((int)this + 0x108) = DAT_006be35c;
    _DAT_006be380 = DAT_006be360;
    _DAT_006be384 = DAT_006be364;
    _DAT_006be388 = DAT_006be368;
    _DAT_006be38c = DAT_006be36c;
    *(undefined4 *)((int)this + 0x24) = DAT_006be358;
    uVar4 = FUN_005fa270((int *)this);
  }
  ExceptionList = pvStack_c;
  return uVar4;
}

