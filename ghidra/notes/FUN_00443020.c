
void __fastcall FUN_00443020(void *param_1)

{
  bool bVar1;
  void *this;
  uint uVar2;
  int iVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ec38;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00606c90(param_1,0x102,0xda,0x2d20,0x2d21,0x66,0x1505,0,0,0,0,0,1);
  this = (void *)FUN_00604500((void *)((int)param_1 + 0x6c),0x66);
  uVar2 = *(uint *)((int)param_1 + 0x124) >> 0x18;
  local_4 = 0;
  if ((uVar2 < 0xf1) || (0xf1 < uVar2)) {
    iVar3 = FUN_004ece60((uint *)((int)param_1 + 0x124));
    if (iVar3 != 0) {
      bVar1 = false;
      goto LAB_004430ba;
    }
  }
  bVar1 = true;
LAB_004430ba:
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((bVar1) && (this != (void *)0x0)) {
    FUN_00603150(this,2,0x2d5c);
    FUN_006030c0(this,2);
  }
  FUN_00606c90(param_1,0x144,0xda,0x2882,0x2883,0x65,0x1954,0,0,0,0,0,1);
  ExceptionList = local_c;
  return;
}

