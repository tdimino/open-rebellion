
void * __thiscall FUN_00413000(void *this,HINSTANCE param_1,uint param_2,int param_3)

{
  undefined4 *this_00;
  int iVar1;
  int iVar2;
  HICON pHVar3;
  HGDIOBJ pvVar4;
  LPBYTE pBVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  uint uVar8;
  int local_24;
  int local_20;
  undefined4 auStack_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062a9a8;
  pvStack_c = ExceptionList;
  if ((param_2 & 1) == 0) {
    local_24 = 0x1e0;
    local_20 = 0x280;
    ExceptionList = &pvStack_c;
  }
  else {
    ExceptionList = &pvStack_c;
    iVar1 = GetSystemMetrics(4);
    iVar2 = GetSystemMetrics(6);
    local_24 = iVar1 + 0x1e0 + iVar2 * 2;
    iVar1 = GetSystemMetrics(5);
    local_20 = iVar1 * 2 + 0x280;
  }
  uVar8 = 0;
  uVar7 = 0;
  uVar6 = 0;
  iVar1 = 0;
  pHVar3 = LoadIconA(param_1,(LPCSTR)0x3e8);
  pvVar4 = GetStockObject(4);
  FUN_005fb850(this,param_1,s_STARWARS_REBELLION_006a8314,
               (-(uint)((param_2 & 1) != 0) & 0xca0000) + 0x86000000,0,0,local_20,local_24,
               (char *)0x0,pvVar4,pHVar3,iVar1,uVar6,uVar7,uVar8);
  uStack_4 = 0;
  FUN_005f4950((void *)((int)this + 0xb4),0);
  uStack_4._0_1_ = 1;
  FUN_005f2f50((undefined4 *)((int)this + 0xd0));
  uStack_4._0_1_ = 2;
  *(undefined4 *)((int)this + 0xe8) = 1;
  *(undefined4 *)((int)this + 0xec) = 0;
  *(undefined4 *)((int)this + 0xf0) = 0;
  *(undefined4 *)((int)this + 0x104) = 7;
  FUN_005fbb20((undefined4 *)((int)this + 0x110));
  this_00 = (undefined4 *)((int)this + 0x148);
  uStack_4._0_1_ = 3;
  FUN_005f2f00(this_00);
  uStack_4._0_1_ = 4;
  *(undefined ***)this = &PTR_FUN_006588f8;
  FUN_005f35b0(auStack_18,s_MinimumReceiveLatency_006a82fc);
  uStack_4._0_1_ = 5;
  FUN_005f4520(this_00,(int)auStack_18,0);
  uStack_4._0_1_ = 4;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_MaximumReceiveLatency_006a82e4);
  uStack_4._0_1_ = 6;
  FUN_005f4520(this_00,(int)auStack_18,0);
  uStack_4._0_1_ = 4;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_MinimumSendLatency_006a82d0);
  uStack_4._0_1_ = 7;
  FUN_005f4520(this_00,(int)auStack_18,0);
  uStack_4._0_1_ = 4;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_MaximumSendLatency_006a82bc);
  uStack_4._0_1_ = 8;
  FUN_005f4520(this_00,(int)auStack_18,0);
  uStack_4._0_1_ = 4;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_DropReceivePercent_006a82a8);
  uStack_4._0_1_ = 9;
  FUN_005f4520(this_00,(int)auStack_18,0);
  uStack_4._0_1_ = 4;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_DropSendPercent_006a8298);
  uStack_4._0_1_ = 10;
  FUN_005f4520(this_00,(int)auStack_18,0);
  uStack_4._0_1_ = 4;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_DupeReceivePercent_006a8284);
  uStack_4._0_1_ = 0xb;
  FUN_005f4520(this_00,(int)auStack_18,0);
  uStack_4._0_1_ = 4;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_DupeSendPercent_006a8274);
  uStack_4._0_1_ = 0xc;
  FUN_005f4520(this_00,(int)auStack_18,0);
  uStack_4._0_1_ = 4;
  FUN_005f2ff0(auStack_18);
  FUN_005f35b0(auStack_18,s_UseWaitForSingleObject_006a825c);
  uStack_4._0_1_ = 0xd;
  FUN_005f4520(this_00,(int)auStack_18,0);
  uStack_4 = CONCAT31(uStack_4._1_3_,4);
  FUN_005f2ff0(auStack_18);
  FUN_005f4610(this_00,param_3);
  *(undefined4 *)((int)this + 0xf4) = 0;
  *(undefined4 *)((int)this + 0xf8) = 0;
  *(undefined4 *)((int)this + 0xfc) = 0;
  *(undefined4 *)((int)this + 0x100) = 0;
  DAT_006be5b4 = LoadCursorA(*(HINSTANCE *)((int)this + 0x1c),(LPCSTR)0x3e9);
  pBVar5 = FUN_00401440();
  FUN_005ff020((char *)pBVar5,s_Rebdlog_dll_006a8250,0xc);
  FUN_005ff020((char *)pBVar5,s_textcomm_dll_006a7edc,1);
  if (pBVar5 != (LPBYTE)0x0) {
    FUN_00618b60(pBVar5);
  }
  FUN_005f49c0(param_1);
  *(undefined4 *)((int)this + 200) = 0;
  *(undefined4 *)((int)this + 0xcc) = 0;
  *(undefined4 *)((int)this + 0xc0) = 0;
  *(undefined4 *)((int)this + 0xb8) = 0;
  *(undefined4 *)((int)this + 0xbc) = 0;
  *(undefined4 *)((int)this + 0xc4) = 0;
  *(undefined4 *)((int)this + 0xdc) = 0;
  *(undefined4 *)((int)this + 0xe4) = 0;
  *(undefined4 *)((int)this + 0x108) = 0;
  *(undefined4 *)((int)this + 0x10c) = 0;
  FUN_005fbbb0((int)this + 0x110);
  uVar6 = FUN_005f41b0(s_SOFTWARE_LucasArts_Entertainment_006a81fc,s_MismatchOpts_006a8240);
  *(undefined4 *)((int)this + 0x154) = uVar6;
  *(uint *)((int)this + 0x128) = param_2;
  *(undefined4 *)((int)this + 0x130) = 0xffffffff;
  *(undefined4 *)((int)this + 0x134) = 0xffffffff;
  uVar6 = FUN_005f41b0(s_SOFTWARE_LucasArts_Entertainment_006a81fc,s_DisplayOpts_006a81f0);
  *(undefined4 *)((int)this + 0x140) = uVar6;
  *(undefined4 *)((int)this + 0x144) = 0x13110000;
  iVar1 = FUN_005f41b0(s_SOFTWARE_LucasArts_Entertainment_006a81fc,s_Volume_006a81e8);
  *(int *)((int)this + 300) = -iVar1;
  *(undefined4 *)((int)this + 0x158) = 0;
  *(undefined4 *)((int)this + 0xb0) = 4;
  ExceptionList = pvStack_c;
  return this;
}

