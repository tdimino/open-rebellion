
void * __thiscall
FUN_0046f140(void *this,undefined4 param_1,int param_2,undefined4 param_3,undefined4 param_4)

{
  int iVar1;
  LPBYTE pBVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006323a6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00606380(this,param_1,0,0,0x1d6,0x14b,param_2,0,7,0,param_3,0,1,0,0);
  local_4 = 0;
  FUN_00601880((void *)((int)this + 0x128),(char *)0x0,0,0,0,0,0,0);
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_0065a300;
  *(undefined4 *)((int)this + 0x50) = 0x46000000;
  *(int *)((int)this + 0x118) = param_2;
  *(undefined4 *)((int)this + 0x174) = param_4;
  *(undefined4 *)((int)this + 0x120) = 0;
  *(undefined4 *)((int)this + 0x124) = 0;
  *(undefined4 *)((int)this + 0x15c) = 0;
  *(undefined4 *)((int)this + 0x160) = 0;
  *(undefined4 *)((int)this + 0x17c) = 0;
  *(undefined4 *)((int)this + 0x168) = 0;
  *(undefined4 *)((int)this + 0x16c) = 0;
  *(undefined4 *)((int)this + 0x184) = 0;
  *(undefined4 *)((int)this + 0x180) = 0x65;
  iVar1 = FUN_005fefd0(10);
  *(int *)((int)this + 0x178) = iVar1;
  if (iVar1 == 0) {
    pBVar2 = FUN_00401440();
    iVar1 = FUN_005ff020((char *)pBVar2,s_gokres_dll_006a8708,10);
    *(int *)((int)this + 0x178) = iVar1;
    if (pBVar2 != (LPBYTE)0x0) {
      FUN_00618b60(pBVar2);
    }
  }
  ExceptionList = local_c;
  return this;
}

