
uint __thiscall
FUN_00567ba0(void *this,int param_1,uint param_2,int param_3,uint *param_4,int *param_5,
            undefined4 *param_6)

{
  uint *puVar1;
  uint uVar2;
  int iVar3;
  undefined4 local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar1 = param_4;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064b258;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_4 = 0;
  *param_5 = 0;
  *param_6 = 0;
  FUN_0052d720(local_1c);
  local_4 = 0;
  param_4 = (uint *)0x0;
  uVar2 = FUN_00567880(this,param_1,param_2,0,0,0,0,local_1c,(int *)&param_4);
  iVar3 = param_3;
  while (((uVar2 != 0 && (iVar3 = iVar3 + 1, iVar3 <= (int)param_4)) && (*puVar1 == 0))) {
    uVar2 = FUN_00567a90(this,param_1,param_2,iVar3,&param_3);
    if ((uVar2 != 0) && (*puVar1 = (uint)(param_3 != 0), (param_3 != 0) != 0)) {
      *param_5 = iVar3;
      *param_6 = *(undefined4 *)(param_3 + 0x54);
    }
  }
  local_4 = 0xffffffff;
  FUN_0052d760(local_1c);
  ExceptionList = local_c;
  return uVar2;
}

