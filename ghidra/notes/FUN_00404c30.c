
void * __thiscall
FUN_00404c30(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6)

{
  LPBYTE pBVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629776;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005ff1c0(this,param_1,param_2,param_3,param_4,param_5,param_6,0,0);
  local_4 = 0;
  FUN_00601500((undefined4 *)((int)this + 0xe4));
  *(undefined ***)this = &PTR_FUN_00658580;
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined4 *)((int)this + 0xb4) = 0;
  *(undefined4 *)((int)this + 0xb0) = 0;
  *(undefined4 *)((int)this + 0xb8) = 0;
  *(undefined4 *)((int)this + 0xbc) = 0;
  *(undefined4 *)((int)this + 0xc0) = 0;
  *(undefined4 *)((int)this + 0xa4) = 0;
  *(undefined4 *)((int)this + 0xa8) = 0;
  *(undefined4 *)((int)this + 0x9c) = 0;
  *(undefined4 *)((int)this + 0xa0) = 0;
  *(undefined4 *)((int)this + 0xac) = 0;
  *(undefined4 *)((int)this + 0xc4) = 0;
  *(undefined4 *)((int)this + 200) = 0;
  *(undefined4 *)((int)this + 0xcc) = 0;
  *(undefined4 *)((int)this + 0xdc) = 0;
  *(undefined4 *)((int)this + 0xd0) = 0;
  *(undefined4 *)((int)this + 0xd4) = 0;
  *(undefined4 *)((int)this + 0xd8) = 0;
  *(undefined4 *)((int)this + 0xec) = 1;
  *(undefined4 *)((int)this + 0xf0) = 1;
  *(undefined4 *)((int)this + 0xf4) = 1;
  *(undefined4 *)((int)this + 0xf8) = 0;
  *(undefined4 *)((int)this + 0x100) = 0;
  *(undefined4 *)((int)this + 0x104) = 0;
  pBVar1 = FUN_00401440();
  FUN_005ff020((char *)pBVar1,s_common_dll_006a7c98,8);
  if (pBVar1 != (LPBYTE)0x0) {
    FUN_00618b60(pBVar1);
  }
  DAT_006b1208 = 0;
  DAT_006b06f4 = this;
  ExceptionList = local_c;
  return this;
}

