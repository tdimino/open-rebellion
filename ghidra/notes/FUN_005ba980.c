
void __thiscall FUN_005ba980(void *this,uint param_1,int param_2)

{
  DWORD DVar1;
  undefined4 *this_00;
  int iVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006535b3;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  DVar1 = timeGetTime();
  this_00 = (undefined4 *)FUN_00618b70(0x44);
  uStack_4 = 0;
  if (this_00 == (undefined4 *)0x0) {
    this_00 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5590(this_00);
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    *(undefined2 *)(this_00 + 10) = 0;
    *(undefined2 *)((int)this_00 + 0x2a) = 0;
    FUN_00610b20(this_00 + 0xb,0,0,0,0,0,0);
    *this_00 = &PTR_FUN_0066c740;
    this_00[9] = DVar1;
    this_00[7] = param_1;
    *(undefined2 *)(this_00 + 10) = 13000;
    if (param_1 < 0x20) {
      *(undefined2 *)((int)this_00 + 0x2a) = 6;
      this_00[8] = 0;
    }
    else {
      *(undefined2 *)((int)this_00 + 0x2a) = 0xe;
      this_00[8] = 0;
    }
  }
  iVar2 = *(int *)((int)this + 0x1c) + 1;
  uStack_4 = 0xffffffff;
  *(int *)((int)this + 0x1c) = iVar2;
  FUN_005f5600(this_00,iVar2);
  FUN_005baa60(this,this_00,(uint)(param_2 == 0));
  ExceptionList = pvStack_c;
  return;
}

