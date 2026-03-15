
void __fastcall FUN_005fa400(void *param_1)

{
  char cVar1;
  int iVar2;
  void *this;
  void *pvVar3;
  DWORD DVar4;
  uint uVar5;
  char *pcVar6;
  undefined4 local_78;
  undefined1 local_74;
  undefined1 auStack_70 [100];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655cfb;
  pvStack_c = ExceptionList;
  if (*(uint *)((int)param_1 + 0x94) < 2) {
    ExceptionList = &pvStack_c;
    DVar4 = timeGetTime();
    *(DWORD *)((int)param_1 + 0xd0) = DVar4;
    *(undefined4 *)((int)param_1 + 0xd4) = 0;
    ExceptionList = pvStack_c;
    return;
  }
  ExceptionList = &pvStack_c;
  iVar2 = FUN_005f88c0((int)param_1);
  FUN_005ae1e0();
  local_78._2_2_ = 0;
  local_78._0_2_ = DAT_006ac754;
  local_74 = 0;
  this = (void *)FUN_00618b70(0x220);
  pvVar3 = (void *)0x0;
  local_4 = 0;
  if (this != (void *)0x0) {
    uVar5 = 0xffffffff;
    pcVar6 = (char *)&local_78;
    do {
      if (uVar5 == 0) break;
      uVar5 = uVar5 - 1;
      cVar1 = *pcVar6;
      pcVar6 = pcVar6 + 1;
    } while (cVar1 != '\0');
    pvVar3 = FUN_005fadf0(this,&local_78,~uVar5,iVar2 << 0x1c | 0xffffff,1,1,1);
  }
  local_4 = 0xffffffff;
  FUN_005f91c0(param_1,(int)pvVar3,1);
  FUN_005f9030(param_1);
  DVar4 = timeGetTime();
  *(DWORD *)((int)param_1 + 0xd0) = DVar4;
  *(undefined4 *)((int)param_1 + 0xd4) = 1;
  FUN_00618ed0(auStack_70,s_Pinging_Players__u_Return_Code___006ac730);
  FUN_005ae1e0();
  FUN_005ae1e0();
  ExceptionList = pvStack_c;
  return;
}

